use crate::communicator::Communicator;
use crate::proto::ProtocolType;
use crate::protocols::Protocol;
use meesign_crypto::proto::{Message, ProtocolGroupInit, ProtocolInit};

pub struct Musig2Group {
    parties: u32,
    threshold: u32,
    round: u16,
}

impl Musig2Group {
    pub fn new(parties: u32, threshold: u32) -> Self {
        Self {
            parties,
            threshold,
            round: 0,
        }
    }
}

impl Protocol for Musig2Group {
    fn initialize(&mut self, communicator: &mut Communicator, _: &[u8]) {
        communicator.set_active_devices();
        let parties = self.parties;
        let threshold = self.threshold;
        communicator.send_all(|idx| {
            (ProtocolGroupInit {
                protocol_type: meesign_crypto::proto::ProtocolType::Musig2 as i32,
                index: idx,
                parties,
                threshold,
            })
            .encode_to_vec()
        });

        self.round = 1;
    }

    fn advance(&mut self, communicator: &mut Communicator) {
        assert!((0..self.last_round()).contains(&self.round));

        communicator.relay();
        self.round += 1;
    }

    fn finalize(&mut self, communicator: &mut Communicator) -> Option<Vec<u8>> {
        assert_eq!(self.last_round(), self.round);
        self.round += 1;
        communicator.get_final_message()
    }

    fn round(&self) -> u16 {
        self.round
    }

    fn last_round(&self) -> u16 {
        2
    }

    fn get_type(&self) -> ProtocolType {
        ProtocolType::Musig2
    }
}

pub struct Musig2Sign {
    round: u16,
}

impl Musig2Sign {
    pub fn new() -> Self {
        Self { round: 0 }
    }
}

impl Protocol for Musig2Sign {
    fn initialize(&mut self, communicator: &mut Communicator, data: &[u8]) {
        communicator.set_active_devices();
        let participant_indices = communicator.get_protocol_indices();
        communicator.send_all(|idx| {
            (ProtocolInit {
                protocol_type: meesign_crypto::proto::ProtocolType::Musig2 as i32,
                indices: participant_indices.clone(),
                index: idx,
                data: Vec::from(data),
            })
            .encode_to_vec()
        });

        self.round = 1;
    }

    fn advance(&mut self, communicator: &mut Communicator) {
        assert!((0..self.last_round()).contains(&self.round));

        communicator.relay();
        self.round += 1;
    }

    fn finalize(&mut self, communicator: &mut Communicator) -> Option<Vec<u8>> {
        assert_eq!(self.last_round(), self.round);
        self.round += 1;
        communicator.get_final_message()
    }

    fn round(&self) -> u16 {
        self.round
    }

    fn last_round(&self) -> u16 {
        3
    }

    fn get_type(&self) -> ProtocolType {
        ProtocolType::Musig2
    }
}
