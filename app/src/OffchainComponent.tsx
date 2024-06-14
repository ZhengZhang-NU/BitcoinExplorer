import React, { useEffect, useState } from 'react';
import { Container, Table } from 'react-bootstrap';

interface OffchainData {
    id: number;
    block_height: number;
    btc_price: number;
    market_sentiment: number;
    volume: number;
    high: number;
    low: number;
    timestamp: string;
}

const OffchainComponent: React.FC = () => {
    const [offchainData, setOffchainData] = useState<OffchainData[]>([]);

    useEffect(() => {
        fetch('http://localhost:8000/offchain-data')
            .then(response => response.json())
            .then(data => setOffchainData(data));
    }, []);

    return (
        <Container>
            <h2 className="my-4">Off-Chain Data</h2>
            <Table striped bordered hover>
                <thead>
                <tr>
                    <th>Block Height</th>
                    <th>BTC Price</th>
                    <th>Market Sentiment</th>
                    <th>Volume</th>
                    <th>High</th>
                    <th>Low</th>
                    <th>Timestamp</th>
                </tr>
                </thead>
                <tbody>
                {offchainData.map(data => (
                    <tr key={data.id}>
                        <td>{data.block_height}</td>
                        <td>{data.btc_price}</td>
                        <td>{data.market_sentiment}</td>
                        <td>{data.volume}</td>
                        <td>{data.high}</td>
                        <td>{data.low}</td>
                        <td>{new Date(data.timestamp).toLocaleString()}</td>
                    </tr>
                ))}
                </tbody>
            </Table>
        </Container>
    );
};

export default OffchainComponent;
