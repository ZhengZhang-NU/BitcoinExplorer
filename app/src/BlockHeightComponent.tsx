// BlockHeightComponent.tsx
import React, { useState, useEffect } from 'react';
import './BlockHeight.css';

interface BlockHeight {
    id: number;
    height: number;
    avg_tx_count: number;
    difficulty: number;
    block_time: number;
    timestamp: string;
    size: number;
    weight: number;
}

const BlockHeightComponent: React.FC = () => {
    const [blockHeights, setBlockHeights] = useState<BlockHeight[]>([]);
    const [searchTerm, setSearchTerm] = useState<string>('');

    useEffect(() => {
        fetch('http://localhost:8000/block-info')
            .then(response => {
                if (!response.ok) {
                    throw new Error(`HTTP error! status: ${response.status}`);
                }
                return response.json();
            })
            .then(data => {
                const uniqueBlocks = Array.from(new Map(data.map((item: BlockHeight) => [item.height, item])).values());
                // @ts-ignore
                setBlockHeights(uniqueBlocks);
            })
            .catch(error => console.error('Error fetching block info:', error));
    }, []);

    const handleSearch = (event: React.ChangeEvent<HTMLInputElement>) => {
        setSearchTerm(event.target.value);
    };

    const filteredBlockHeights = blockHeights.filter(block =>
        block.height.toString().includes(searchTerm) ||
        block.id.toString().includes(searchTerm) ||
        block.timestamp.includes(searchTerm)
    );

    return (
        <div className="block-height">
            <div className="input-group mb-3">
                <input
                    type="text"
                    className="form-control"
                    placeholder="Search for block or transaction"
                    value={searchTerm}
                    onChange={handleSearch}
                />
                <div className="input-group-append">
                    <button className="btn btn-outline-secondary" type="button">Search</button>
                </div>
            </div>
            <h2>Latest Blocks</h2>
            <table className="table table-dark table-striped">
                <thead>
                <tr>
                    <th>Height</th>
                    <th>Timestamp</th>
                    <th>Avg TX Count</th>
                    <th>Difficulty</th>
                    <th>Size (KB)</th>
                    <th>Weight (KWU)</th>
                </tr>
                </thead>
                <tbody>
                {filteredBlockHeights.map(block => (
                    <tr key={block.id}>
                        <td>{block.height}</td>
                        <td>{block.timestamp}</td>
                        <td>{block.avg_tx_count}</td>
                        <td>{block.difficulty}</td>
                        <td>{(block.size / 1000).toFixed(3)}</td>
                        <td>{block.weight}</td>
                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    );
}

export default BlockHeightComponent;
