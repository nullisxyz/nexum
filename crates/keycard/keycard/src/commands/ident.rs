use nexum_apdu_macros::apdu_pair;

use nexum_apdu_globalplatform::constants::status::*;
use rand::RngCore;

use crate::types::Signature;

use super::CLA_GP;

apdu_pair! {
    /// IDENT command for Keycard
    pub struct Ident {
        command {
            cla: CLA_GP,
            ins: 0x14,

            builders {
                /// Create an IDENT command with the nominated challenge
                pub fn with_challenge(challenge: &[u8; 32]) -> Self {
                    Self::new(0x00, 0x00).with_data(challenge.to_vec()).with_le(0)
                }

                /// Create an IDENT command with a random 256-bit challenge
                pub fn with_random_challenge() -> Self {
                    let mut rng = rand::rng();
                    let mut challenge = [0u8; 32];
                    rng.fill_bytes(&mut challenge);
                    Self::with_challenge(&challenge)
                }
            }
        }

        response {
            ok {
                /// Success response
                #[sw(SW_NO_ERROR)]
                Success {
                    /// The signature from the card of the challenge
                    signature: crate::types::Signature,
                }
            }

            errors {
                /// Wrong data
                #[sw(SW_WRONG_DATA)]
                #[error("Wrong data")]
                WrongData,
            }

            custom_parse = |response: &nexum_apdu_core::Response| -> Result<IdentOk, IdentError> {
                match response.status() {
                    SW_NO_ERROR => match response.payload() {
                        Some(payload) => Ok(IdentOk::Success {
                            signature: Signature::try_from(payload.as_ref())
                                .map_err(|_| Error::ParseError("Unable to parse signature"))?,
                        }),
                        None => Err(Error::ParseError("No payload data"))?,
                    },
                    SW_WRONG_DATA => Err(IdentError::WrongData),
                    _ => Err(IdentError::Unknown { sw1: response.status().sw1, sw2: response.status().sw2 }),
                }
            }
        }
    }
}
