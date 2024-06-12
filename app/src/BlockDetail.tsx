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

    if (loading) {
        return <div>Loading...</div>;
    }

    if (!blockDetail) {
        return <div>No block details available</div>;
    }

    return (
        <div>
            <h2>Block Details - Height {blockDetail.block_info.height}</h2>
            <div>
                <h3>Transactions</h3>
                {blockDetail.transactions.length > 0 ? (
                    blockDetail.transactions.map((tx) => (
                        <div key={tx.id} className="transaction">
                            <h4>Transaction {tx.hash}</h4>
                            <p>BTC: {tx.btc}</p>
                            <p>Fee: {tx.fee}</p>
                            <p>Time: {new Date(tx.time * 1000).toLocaleString()}</p>
                            <div>
                                <h5>Inputs</h5>
                                {blockDetail.inputs
                                    .filter((input) => input.transaction_id === tx.id)
                                    .map((input) => (
                                        <p key={input.id}>
                                            {input.previous_output} - {input.value} BTC
                                        </p>
                                    ))}
                            </div>
                            <div>
                                <h5>Outputs</h5>
                                {blockDetail.outputs
                                    .filter((output) => output.transaction_id === tx.id)
                                    .map((output) => (
                                        <p key={output.id}>
                                            {output.address} - {output.value} BTC
                                        </p>
                                    ))}
                            </div>
                        </div>
                    ))
                ) : (
                    <p>No transactions found for this block.</p>
                )}
            </div>
        </div>
    );
};

export default BlockDetail;
