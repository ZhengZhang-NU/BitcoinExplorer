import React, { useState, useEffect } from 'react';

const BlockHeight: React.FC = () => {
    const [blockHeight, setBlockHeight] = useState<number | null>(null);

    useEffect(() => {

        fetchBlockHeight();
    }, []);

    const fetchBlockHeight = async () => {
        try {

            const response = await fetch('http://localhost:8000/block-height'); // 假设后端 API 地址为 http://localhost:8000/block-height
            const data = await response.json();
            setBlockHeight(data.height);
        } catch (error) {
            console.error('Error fetching block height:', error);
        }
    };

    return (
        <div>
            <h1>Bitcoin Explorer</h1>
            <p>Block Height: {blockHeight}</p>
        </div>
    );
};

export default BlockHeight;