// DifficultyAdjustment.tsx
import React, { useEffect, useState } from 'react';

interface DifficultyAdjustmentData {
    timestamp: number;
    difficulty: number;
}

const DifficultyAdjustment: React.FC = () => {
    const [difficulties, setDifficulties] = useState<DifficultyAdjustmentData[]>([]);

    useEffect(() => {
        fetch('/api/difficulty-adjustment')
            .then(response => response.json())
            .then(data => setDifficulties(data))
            .catch(error => console.error('Error fetching difficulty adjustment data:', error));
    }, []);

    return (
        <div>
            <h1>Difficulty Adjustment</h1>
            {difficulties.length > 0 ? (
                <table>
                    <thead>
                    <tr>
                        <th>Timestamp</th>
                        <th>Difficulty</th>
                    </tr>
                    </thead>
                    <tbody>
                    {difficulties.map((difficulty, index) => (
                        <tr key={index}>
                            <td>{new Date(difficulty.timestamp * 1000).toLocaleString()}</td>
                            <td>{difficulty.difficulty}</td>
                        </tr>
                    ))}
                    </tbody>
                </table>
            ) : (
                <p>Loading...</p>
            )}
        </div>
    );
};

export default DifficultyAdjustment;
