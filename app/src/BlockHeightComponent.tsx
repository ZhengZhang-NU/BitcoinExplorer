import React, { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { Container, Table, Pagination } from 'react-bootstrap';

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
    const [blockData, setBlockData] = useState<BlockInfo[]>([]);
    const [currentPage, setCurrentPage] = useState(1);
    const blocksPerPage = 10;

    useEffect(() => {
        fetch('http://localhost:8000/block-info')
            .then(response => response.json())
            .then(data => setBlockData(data));
    }, []);

    const indexOfLastBlock = currentPage * blocksPerPage;
    const indexOfFirstBlock = indexOfLastBlock - blocksPerPage;
    const currentBlocks = blockData.slice(indexOfFirstBlock, indexOfLastBlock);

    const paginate = (pageNumber: number) => setCurrentPage(pageNumber);

    return (
        <Container>
            <h2 className="my-4">Latest Blocks</h2>
            <Table striped bordered hover>
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
                {currentBlocks.map(block => (
                    <tr key={block.height}>
                        <td>
                            <Link to={`/block/${block.height}`}>{block.height}</Link>
                        </td>
                        <td>{block.avg_tx_count}</td>
                        <td>{block.difficulty}</td>
                        <td>{block.block_time}</td>
                        <td>{new Date(block.timestamp).toLocaleString()}</td>
                        <td>{block.size}</td>
                        <td>{block.weight}</td>
                    </tr>
                ))}
                </tbody>
            </Table>
            <Pagination>
                <Pagination.Prev onClick={() => paginate(currentPage - 1)} disabled={currentPage === 1} />
                <Pagination.Item>{currentPage}</Pagination.Item>
                <Pagination.Next onClick={() => paginate(currentPage + 1)} disabled={indexOfLastBlock >= blockData.length} />
            </Pagination>
        </Container>
    );
};

export default BlockHeightComponent;
