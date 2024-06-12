import React, { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import "./BlockHeightComponent.css";

interface BlockInfo {
    height: number;
    avg_tx_count: number;
    difficulty: number;
    block_time: number;
    timestamp: string;
    size: number;
    weight: number;
}

const BlockHeightComponent: React.FC = () => {
    const [blockInfo, setBlockInfo] = useState<BlockInfo[]>([]);
    const navigate = useNavigate();

    useEffect(() => {
        const fetchData = async () => {
            try {
                const response = await fetch("http://localhost:8000/block-info");
                const data = await response.json();
                // Filter unique blocks based on height
                const uniqueData = data.filter((item: BlockInfo, index: number, self: BlockInfo[]) =>
                    index === self.findIndex((t) => t.height === item.height)
                );
                setBlockInfo(uniqueData);
            } catch (error) {
                console.error("Failed to fetch block info:", error);
            }
        };
        fetchData();
    }, []);

    const handleRowClick = (height: number) => {
        navigate(`/block/${height}`);
    };

    return (
        <div className="block-height-component">
            <h2>Latest Blocks</h2>
            <table className="block-table">
                <thead>
                <tr>
                    <th>Height</th>
                    <th>Avg TX Count</th>
                    <th>Difficulty</th>
                    <th>Block Time</th>
                    <th>Timestamp</th>
                    <th>Size (KB)</th>
                    <th>Weight (KWU)</th>
                </tr>
                </thead>
                <tbody>
                {blockInfo.map((block) => (
                    <tr key={block.height} onClick={() => handleRowClick(block.height)}>
                        <td>{block.height}</td>
                        <td>{block.avg_tx_count}</td>
                        <td>{block.difficulty}</td>
                        <td>{block.block_time}</td>
                        <td>{block.timestamp}</td>
                        <td>{block.size}</td>
                        <td>{block.weight}</td>
                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    );
};

export default BlockHeightComponent;
