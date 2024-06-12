import React, { useEffect, useState } from "react";

interface TransactionFee {
    priority: string;
    fee: number;
}

const TransactionFees: React.FC = () => {
    const [fees, setFees] = useState<TransactionFee[]>([]);

    useEffect(() => {
        const fetchFees = async () => {
            const response = await fetch("http://localhost:8000/transaction-fees");
            const data = await response.json();
            setFees(data);
        };
        fetchFees();
    }, []);

    return (
        <div>
            <h2>Transaction Fees</h2>
            <table>
                <thead>
                <tr>
                    <th>Priority</th>
                    <th>Fee (sat/vB)</th>
                </tr>
                </thead>
                <tbody>
                {fees.map((fee) => (
                    <tr key={fee.priority}>
                        <td>{fee.priority}</td>
                        <td>{fee.fee}</td>
                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    );
};

export default TransactionFees;
