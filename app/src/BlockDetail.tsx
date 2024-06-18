import React, { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { Container, Card, Row, Col, Button } from 'react-bootstrap';
import "./BlockDetail.css";

interface TransactionInput {
    id: number;
    transaction_id: number;
    previous_output: string;
    value: number;
}

interface TransactionOutput {
    id: number;
    transaction_id: number;
    address: string;
    value: number;
}

interface Transaction {
    id: number;
    block_height: number;
    hash: string;
    btc: number;
    fee: number;
    time: number;
}

interface BlockDetailData {
    block_info: {
        id: number;
        height: number;
        avg_tx_count: number;
        difficulty: number;
        block_time: number;
        timestamp: string;
        size: number;
        weight: number;
    };
    transactions: Transaction[];
    inputs: TransactionInput[];
    outputs: TransactionOutput[];
}

const BlockDetail: React.FC = () => {
    const { height } = useParams<{ height: string }>();
    const [blockDetail, setBlockDetail] = useState<BlockDetailData | null>(null);
    const [loading, setLoading] = useState(true);
    const [currentPage, setCurrentPage] = useState(1);
    const [btcToUsdRate, setBtcToUsdRate] = useState<number | null>(null);
    const [isUsd, setIsUsd] = useState(false);
    const transactionsPerPage = 10;

    useEffect(() => {
        const fetchBlockDetail = async () => {
            try {
                const response = await fetch(`http://localhost:8000/block/${height}`);
                if (!response.ok) {
                    throw new Error("Failed to fetch block detail");
                }
                const data = await response.json();
                setBlockDetail(data);
            } catch (error) {
                console.error("Error fetching block detail:", error);
            } finally {
                setLoading(false);
            }
        };

        const fetchBtcToUsdRate = async () => {
            try {
                const response = await fetch('https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd');
                if (!response.ok) {
                    throw new Error("Failed to fetch BTC to USD rate");
                }
                const data = await response.json();
                setBtcToUsdRate(data.bitcoin.usd);
            } catch (error) {
                console.error("Error fetching BTC to USD rate:", error);
            }
        };

        fetchBlockDetail();
        fetchBtcToUsdRate();
    }, [height]);

    const toggleCurrency = () => {
        setIsUsd(!isUsd);
    };

    const paginate = (direction: string) => {
        if (direction === "next" && blockDetail && currentPage < Math.ceil(blockDetail.transactions.length / transactionsPerPage)) {
            setCurrentPage(currentPage + 1);
        } else if (direction === "prev" && currentPage > 1) {
            setCurrentPage(currentPage - 1);
        }
    };

    if (loading) {
        return <div>Loading...</div>;
    }

    if (!blockDetail) {
        return <div>No block details available</div>;
    }

    const indexOfLastTransaction = currentPage * transactionsPerPage;
    const indexOfFirstTransaction = indexOfLastTransaction - transactionsPerPage;
    const currentTransactions = blockDetail.transactions.slice(indexOfFirstTransaction, indexOfLastTransaction);

    return (
        <Container>
            <div className="currency-toggle">
                <Button onClick={toggleCurrency} className="currency-button">
                    {isUsd ? 'Switch to BTC' : 'Switch to USD'}
                </Button>
            </div>
            <h2 className="my-4">Block Details - Height {blockDetail.block_info.height}</h2>
            <div className="transactions">
                <h3>Transactions</h3>
                {currentTransactions.length > 0 ? (
                    currentTransactions.map((tx) => (
                        <Card key={tx.id} className="transaction-card mb-4">
                            <Card.Header>Transaction {tx.hash}</Card.Header>
                            <Card.Body>
                                <Card.Text>
                                    BTC: {isUsd ? `$ ${(tx.btc / 100000000 * (btcToUsdRate || 0)).toFixed(2)} USD` : `${(tx.btc / 100000000).toFixed(8)} BTC`}
                                </Card.Text>
                                <Card.Text>
                                    Fee: {isUsd ? `$ ${(tx.fee / 100000000 * (btcToUsdRate || 0)).toFixed(2)} USD` : `${(tx.fee / 100000000).toFixed(8)} BTC`}
                                </Card.Text>
                                <Card.Text>Time: {new Date(tx.time * 1000).toLocaleString()}</Card.Text>
                                <Row className="transaction-io">
                                    <Col md={6}>
                                        <h5>Inputs</h5>
                                        {blockDetail.inputs
                                            .filter((input) => input.transaction_id === tx.id)
                                            .map((input) => (
                                                <p key={input.id} className="input-output">
                                                    {input.previous_output}
                                                    {isUsd ? `$ ${(input.value / 100000000 * (btcToUsdRate || 0)).toFixed(2)} USD` : `${(input.value / 100000000).toFixed(8)} BTC`}
                                                </p>
                                            ))}
                                    </Col>
                                    <Col md={6}>
                                        <h5>Outputs</h5>
                                        {blockDetail.outputs
                                            .filter((output) => output.transaction_id === tx.id)
                                            .map((output) => (
                                                <p key={output.id} className="input-output">
                                                    {output.address || " "}
                                                    {isUsd ? `$ ${(output.value / 100000000 * (btcToUsdRate || 0)).toFixed(2)} USD` : `${(output.value / 100000000).toFixed(8)} BTC`}
                                                </p>
                                            ))}
                                    </Col>
                                </Row>
                            </Card.Body>
                        </Card>
                    ))
                ) : (
                    <p>No transactions found for this block.</p>
                )}
                <div className="pagination">
                    <Button onClick={() => paginate("prev")} disabled={currentPage === 1}>Previous</Button>
                    <Button onClick={() => paginate("next")} disabled={currentPage >= Math.ceil(blockDetail.transactions.length / transactionsPerPage)}>Next</Button>
                </div>
            </div>
        </Container>
    );
};

export default BlockDetail;
