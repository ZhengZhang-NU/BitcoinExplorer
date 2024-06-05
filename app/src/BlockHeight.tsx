import React, { useState, useEffect } from 'react';
import './BlockHeight.css';

interface BlockHeight {
    id: number;
    height: number;
}

const BlockHeight: React.FC = () => {
    const [blockHeights, setBlockHeights] = useState<BlockHeight[]>([]);

    useEffect(() => {
        const fetchBlockHeights = async () => {
            try {
                const response = await fetch('http://localhost:8000/block-height');
                if (!response.ok) {
                    throw new Error(`HTTP error! status: ${response.status}`);
                }
                const data = await response.json() as BlockHeight[];
                setBlockHeights(data);
            } catch (error) {
                console.error('Error fetching block heights:', error);
            }
        };

        const intervalId = setInterval(fetchBlockHeights, 10000); // 每10秒获取一次
        fetchBlockHeights(); // 初始调用

        return () => clearInterval(intervalId); // 清除定时器
    }, []);

    return (
        <div>
            <h1>Bitcoin Explorer</h1>
            <table>
                <thead>
                <tr>
                    <th>ID</th>
                    <th>Height</th>
                </tr>
                </thead>
                <tbody>
                {blockHeights.map((blockHeight) => (
                    <tr key={blockHeight.id}>
                        <td>{blockHeight.id}</td>
                        <td>{blockHeight.height}</td>
                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    );
};

export default BlockHeight;
