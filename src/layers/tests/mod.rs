#[cfg(test)]
mod integration_tests {
    use crate::layers::{
        l1_orchestration::OrchestrationLayer,
        l2_mainnet::MainnetLayer,
        l2_sidenet::SidenetLayer,
        l3_private::PrivateChainLayer,
    };
    use crate::blockchain::core::Block;

    const PRECISION: u8 = 20;

    #[test]
    fn test_layer_interaction() {
        // Initialize layers
        let mut mainnet = MainnetLayer::new(PRECISION);
        let mut sidenet = SidenetLayer::new(PRECISION);
        
        // Add test data to mainnet
        let mainnet_data = b"mainnet_test_data";
        let mainnet_proof = b"mainnet_test_proof";
        let mainnet_hash = mainnet.process_block(mainnet_data, mainnet_proof)
            .expect("Failed to process mainnet block");

        // Add test data to sidenet
        let sidenet_data = b"sidenet_test_data";
        let sidenet_proof = b"sidenet_test_proof";
        let sidenet_hash = sidenet.process_block(sidenet_data, sidenet_proof)
            .expect("Failed to process sidenet block");

        // Anchor sidenet to mainnet
        assert!(sidenet.anchor_to_mainnet(mainnet_hash).is_ok());
        assert_eq!(sidenet.get_latest_anchor(), Some(mainnet_hash));
    }

    #[test]
    fn test_multi_layer_synchronization() {
        let mut orchestration = OrchestrationLayer::new(PRECISION);
        let mut mainnet = MainnetLayer::new(PRECISION);
        let mut sidenet = SidenetLayer::new(PRECISION);
        let mut private_chain = PrivateChainLayer::new(
            Default::default(),
            PRECISION,
        );

        // Process blocks on each layer
        let test_data = b"test_synchronization";
        let test_proof = b"test_proof";
        
        // Mainnet block
        let mainnet_hash = mainnet.process_block(test_data, test_proof)
            .expect("Failed to process mainnet block");

        // Sidenet block and anchor
        let sidenet_hash = sidenet.process_block(test_data, test_proof)
            .expect("Failed to process sidenet block");
        assert!(sidenet.anchor_to_mainnet(mainnet_hash).is_ok());

        // Private chain block and anchor
        let private_sig = [0u8; 64]; // Mock signature
        let private_hash = private_chain.process_block(test_data, test_proof, &private_sig)
            .expect("Failed to process private chain block");
        assert!(private_chain.anchor_to_mainnet(mainnet_hash).is_ok());

        // Verify states
        assert_eq!(mainnet.height(), 1);
        assert_eq!(sidenet.height(), 1);
        assert_eq!(private_chain.height(), 1);
        
        // Verify anchoring
        assert_eq!(sidenet.get_latest_anchor(), Some(mainnet_hash));
        assert_eq!(private_chain.get_latest_anchor(), Some(mainnet_hash));
    }

    #[test]
    fn test_layer_security() {
        let mut mainnet = MainnetLayer::new(PRECISION);
        let mut sidenet = SidenetLayer::new(PRECISION);

        // Test invalid data handling
        assert!(mainnet.process_block(&[], &[]).is_err());
        assert!(sidenet.process_block(&[], &[]).is_err());

        // Test valid data handling
        let valid_data = b"valid_test_data";
        let valid_proof = b"valid_test_proof";

        assert!(mainnet.process_block(valid_data, valid_proof).is_ok());
        assert!(sidenet.process_block(valid_data, valid_proof).is_ok());
    }
}
