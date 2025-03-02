#!/bin/bash

echo "Testing Quantum Metaverse Blockchain Components..."
echo

echo "1. Testing Node Status..."
curl -s -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"status","params":[],"id":1}' http://localhost:8545
echo -e "\n"

echo "2. Testing System Metrics..."
curl -s -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"getMetrics","params":[],"id":1}' http://localhost:8545
echo -e "\n"

echo "3. Testing Quantum State..."
curl -s -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"getQuantumState","params":[],"id":1}' http://localhost:8545
echo -e "\n"

echo "4. Testing AI Governance..."
curl -s -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"getAIDecisions","params":[],"id":1}' http://localhost:8545
echo -e "\n"

echo "5. Testing Security..."
curl -s -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"security_test","params":[],"id":1}' http://localhost:8545
echo -e "\n"

echo "6. Testing Quantum Attack Simulation..."
curl -s -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"quantum_attack_simulation","params":[],"id":1}' http://localhost:8545
echo -e "\n"

echo "7. Testing Network Security Audit..."
curl -s -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"network_security_audit","params":[],"id":1}' http://localhost:8545
echo -e "\n"

echo "8. Testing Orchestration Metrics..."
curl -s -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"getOrchestrationMetrics","params":[],"id":1}' http://localhost:8545
echo -e "\n"

echo "9. Testing Quantum State Recording..."
curl -s -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"recordQuantumState","params":[{"observer_id":"0x0000000067c4124a000000000000000000000000000000000000000000000000","quantum_state":"0x0123456789abcdef","reality_layer":1}],"id":1}' http://localhost:8545
echo -e "\n"

echo "10. Testing Stress Test..."
curl -s -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"stress_test","params":[],"id":1}' http://localhost:8545
echo -e "\n"
