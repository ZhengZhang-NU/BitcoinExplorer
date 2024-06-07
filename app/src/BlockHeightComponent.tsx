import React, { useState, useEffect } from 'react';
import './BlockHeight.css';


interface BlockHeight {
    id: number;
    height: number;
}

const BlockHeightComponent: React.FC = () => {
    const [blockHeights, setBlockHeights] = useState<BlockHeight[]>([]);

    useEffect(() => {
        const fetchBlockHeights = async () => {
            try {
                const response = await fetch('http://localhost:8000/block-height');
                if (!response.ok) {
                    throw new Error(`HTTP error! status: ${response.status}`);
                }
                const data = await response.json();
                setBlockHeights(data);
            } catch (error) {
                console.error('Error fetching block heights:', error);
            }
        };

        const intervalId = setInterval(fetchBlockHeights, 10000);
        fetchBlockHeights();

        return () => clearInterval(intervalId);
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
                {blockHeights.map((block) => (
                    <tr key={block.id}>
                        <td>{block.id}</td>
                        <td>{block.height}</td>
                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    );
};

export default BlockHeightComponent;
