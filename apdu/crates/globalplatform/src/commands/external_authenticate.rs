//! EXTERNAL AUTHENTICATE command for GlobalPlatform
//!
//! This command is used to authenticate the host to the card
//! and establish a secure channel.

use nexum_apdu_macros::apdu_pair;

use crate::{constants::*, crypto::calculate_cryptogram};

apdu_pair! {
    /// EXTERNAL AUTHENTICATE command for GlobalPlatform
    pub struct ExternalAuthenticate {
        command {
            cla: cla::MAC,
            ins: ins::EXTERNAL_AUTHENTICATE,

            builders {
                /// Create a new EXTERNAL AUTHENTICATE command with host cryptogram
                pub fn with_host_cryptogram(host_cryptogram: impl Into<bytes::Bytes>) -> Self {
                    Self::new(external_auth_p1::CMAC, 0x00).with_data(host_cryptogram.into())
                }

                /// Create host cryptogram and command for SCP02
                pub fn from_challenges(
                    enc_key: &cipher::Key<crate::crypto::Scp02> ,
                    sequence_counter: &[u8; 2],
                    card_challenge: &[u8; 6],
                    host_challenge: &[u8; 8],
                ) -> Self {
                    let host_cryptogram = calculate_cryptogram(
                        enc_key, sequence_counter, card_challenge, host_challenge, true);
                    Self::with_host_cryptogram(host_cryptogram.to_vec())
                }
            }
        }

        response {
            ok {
                /// Success response
                #[sw(SW_NO_ERROR)]
                Success,
            }

            errors {
                /// Security status not satisfied
                #[sw(SW_SECURITY_STATUS_NOT_SATISFIED)]
                #[error("Security status not satisfied")]
                SecurityStatusNotSatisfied,

                /// Record not found
                #[sw(SW_RECORD_NOT_FOUND)]
                #[error("Record not found")]
                RecordNotFound,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::Scp02;

    use super::*;
    use bytes::Bytes;
    use cipher::Key;
    use hex_literal::hex;
    use nexum_apdu_core::ApduCommand;

    #[test]
    fn test_external_authenticate_command() {
        let cryptogram = hex!("7702AC6CE46A47F0");
        let cmd = ExternalAuthenticateCommand::with_host_cryptogram(cryptogram.to_vec());

        assert_eq!(cmd.class(), cla::MAC);
        assert_eq!(cmd.instruction(), ins::EXTERNAL_AUTHENTICATE);
        assert_eq!(cmd.p1(), external_auth_p1::CMAC);
        assert_eq!(cmd.p2(), 0x00);
        assert_eq!(cmd.data(), Some(cryptogram.as_ref()));

        // Test command serialization
        let raw = cmd.to_bytes();
        assert_eq!(raw.as_ref(), hex!("8482010008 7702AC6CE46A47F0"));
    }

    #[test]
    fn test_from_challenges() {
        let enc_key = hex!("8D289AFE0AB9C45B1C76DEEA182966F4");
        let enc_key = Key::<Scp02>::clone_from_slice(&enc_key);
        let sequence_counter = hex!("000f");
        let card_challenge = hex!("3fd65d4d6e45");
        let host_challenge = hex!("cf307b6719bf224d");

        let cmd = ExternalAuthenticateCommand::from_challenges(
            &enc_key,
            &sequence_counter,
            &card_challenge,
            &host_challenge,
        );

        assert_eq!(cmd.class(), cla::MAC);
        assert_eq!(cmd.instruction(), ins::EXTERNAL_AUTHENTICATE);
        assert_eq!(cmd.p1(), external_auth_p1::CMAC);
        assert_eq!(cmd.p2(), 0x00);

        // The exact cryptogram will depend on the MAC implementation
        assert_eq!(cmd.data().unwrap().len(), 8);
    }

    #[test]
    fn test_external_authenticate_response() {
        // Test successful response
        let response_data = Bytes::from_static(&hex!("9000"));
        let ok = ExternalAuthenticateCommand::parse_response_raw(response_data).unwrap();
        assert!(matches!(ok, ExternalAuthenticateOk::Success));

        // Test error response
        let error_data = Bytes::from_static(&hex!("6982"));
        let error_result = ExternalAuthenticateCommand::parse_response_raw(error_data).unwrap_err();
        assert!(matches!(
            error_result,
            ExternalAuthenticateError::SecurityStatusNotSatisfied
        ));
    }
}
