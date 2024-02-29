use std::fmt::Display;
use serde::{Deserialize, Serialize};
use xelis_he::{
    DecryptHandle,
    ElGamalCiphertext,
    Identity,
    PedersenCommitment,
    RistrettoPoint
};
use crate::serializer::{Serializer, ReaderError, Reader, Writer};
use super::Ciphertext;

#[derive(Clone, Deserialize, Serialize)]
pub struct VersionedBalance {
    // Output balance is used in case of multi TXs not in same block
    // If you build several TXs at same time but are not in the same block,
    // and a incoming tx happen we need to keep track of the output balance
    output_balance: Option<Ciphertext>,
    // Final user balance that contains outputs and inputs balance
    // This is the balance shown to a user and used to build TXs
    final_balance: Ciphertext,
    previous_topoheight: Option<u64>,
}

impl VersionedBalance {
    pub const fn new(balance: Ciphertext, previous_topoheight: Option<u64>) -> Self {
        Self {
            output_balance: None,
            final_balance: balance,
            previous_topoheight
        }
    }

    pub fn zero() -> Self {
        let zero = ElGamalCiphertext::new(
            PedersenCommitment::from_point(RistrettoPoint::identity()),
            DecryptHandle::from_point(RistrettoPoint::identity())
        );

        Self {
            output_balance: None,
            final_balance: Ciphertext::Decompressed(zero),
            previous_topoheight: None
        }
    }

    pub fn get_balance(&self) -> &Ciphertext {
        &self.final_balance
    }

    pub fn get_mut_balance(&mut self) -> &mut Ciphertext {
        &mut self.final_balance
    }

    pub fn take_balance(self) -> Ciphertext {
        self.final_balance
    }

    pub fn set_output_balance(&mut self, value: Ciphertext) {
        self.output_balance = Some(value);
    }

    pub fn get_output_balance(&self) -> Option<&Ciphertext> {
        self.output_balance.as_ref()
    }

    pub fn get_mut_output_balance(&mut self) -> Option<&mut Ciphertext> {
        self.output_balance.as_mut()
    }

    pub fn set_balance(&mut self, value: Ciphertext) {
        self.final_balance = value;
    }

    pub fn add_plaintext_to_balance(&mut self, _value: u64) {
        // self.balance + Scalar::from(value); 
    }

    pub fn get_previous_topoheight(&self) -> Option<u64> {
        self.previous_topoheight        
    }

    pub fn set_previous_topoheight(&mut self, previous_topoheight: Option<u64>) {
        self.previous_topoheight = previous_topoheight;
    }
}

impl Default for VersionedBalance {
    fn default() -> Self {
        Self::zero()
    }
}

impl Display for VersionedBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let compress = self.final_balance.compress();
        write!(f, "Balance[ciphertext[{}, {}], previous: {:?}", hex::encode(compress.0[0]), hex::encode(compress.0[1]), self.previous_topoheight)
    }
}

impl Serializer for VersionedBalance {
    fn write(&self, writer: &mut Writer) {
        self.final_balance.write(writer);
        if let Some(topo) = &self.previous_topoheight {
            writer.write_u64(topo);
        }
        if let Some(output) = &self.output_balance {
            output.write(writer);
        }
    }

    fn read(reader: &mut Reader) -> Result<Self, ReaderError> {
        let final_balance = Ciphertext::read(reader)?;
        let (previous_topoheight, output_balance) = if reader.size() == 0 {
            (None, None)
        } else {
            if reader.size() == 8 {
                (Some(reader.read_u64()?), None)
            } else {
                (Some(reader.read_u64()?), Some(Ciphertext::read(reader)?))
            }
        };

        Ok(Self {
            output_balance,
            final_balance,
            previous_topoheight
        })
    }

    fn size(&self) -> usize {
        self.final_balance.size()
        + if let Some(topoheight) = self.previous_topoheight { topoheight.size() } else { 0 }
        + if let Some(output_balance) = &self.output_balance { output_balance.size() } else { 0 }
    }
}