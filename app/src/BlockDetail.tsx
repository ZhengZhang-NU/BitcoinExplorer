import React, { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
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
    const transactionsPerPage = 20;

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
        fetchBlockDetail();
    }, [height]);

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
        <div>
            <h2>Block Details - Height {blockDetail.block_info.height}</h2>
            <div>
                <h3>Transactions</h3>
                {currentTransactions.length > 0 ? (
                    currentTransactions.map((tx) => (
                        <div key={tx.id} className="transaction-card">
                            <h4>Transaction {tx.hash}</h4>
                            <p>BTC: {tx.btc}</p>
                            <p>Fee: {tx.fee}</p>
                            <p>Time: {new Date(tx.time * 1000).toLocaleString()}</p>
                            <div className="transaction-io">
                                <div className="inputs">
                                    <h5>Inputs</h5>
                                    {blockDetail.inputs
                                        .filter((input) => input.transaction_id === tx.id)
                                        .map((input) => (
                                            <p key={input.id} className="input-output">
                                                {input.previous_output} - {input.value} BTC
                                            </p>
                                        ))}
                                </div>
                                <div className="outputs">
                                    <h5>Outputs</h5>
                                    {blockDetail.outputs
                                        .filter((output) => output.transaction_id === tx.id)
                                        .map((output) => (
                                            <p key={output.id} className="input-output">
                                                {output.address} - {output.value} BTC
                                            </p>
                                        ))}
                                </div>
                            </div>
                        </div>
                    ))
                ) : (
                    <p>No transactions found for this block.</p>
                )}
                <div className="pagination">
                    <button onClick={() => paginate("prev")} disabled={currentPage === 1}>Previous</button>
                    <button onClick={() => paginate("next")} disabled={currentPage >= Math.ceil(blockDetail.transactions.length / transactionsPerPage)}>Next</button>
                </div>
            </div>
        </div>
    );
};

export default BlockDetail;
