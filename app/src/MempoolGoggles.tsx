import React, { useEffect, useState } from "react";
import { Treemap, Tooltip } from "recharts";

interface MempoolTransaction {
    size: number;
}

const MempoolGoggles: React.FC = () => {
    const [transactions, setTransactions] = useState<MempoolTransaction[]>([]);

    useEffect(() => {
        const fetchTransactions = async () => {
            const response = await fetch("http://localhost:8000/mempool-transactions");
            const data = await response.json();
            setTransactions(data);
        };
        fetchTransactions();
    }, []);

    return (
        <div>
            <h2>Mempool Goggles™</h2>
            <Treemap
                width={400}
                height={400}
                data={transactions}
                dataKey="size"
                stroke="#fff"
                fill="#8884d8"
            >
                <Tooltip />
            </Treemap>
        </div>
    );
};

export default MempoolGoggles;
