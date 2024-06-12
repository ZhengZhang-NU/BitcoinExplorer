import React, { useEffect, useState } from "react";

interface DifficultyAdjustment {
    adjustment: number;
    next_adjustment: string;
}

const DifficultyAdjustment: React.FC = () => {
    const [adjustment, setAdjustment] = useState<DifficultyAdjustment | null>(null);

    useEffect(() => {
        const fetchAdjustment = async () => {
            const response = await fetch("http://localhost:8000/difficulty-adjustment");
            const data = await response.json();
            setAdjustment(data);
        };
        fetchAdjustment();
    }, []);

    if (!adjustment) {
        return <div>Loading...</div>;
    }

    return (
        <div>
            <h2>Difficulty Adjustment</h2>
            <div>
                <p>Current Adjustment: {adjustment.adjustment}%</p>
                <p>Next Adjustment in: {adjustment.next_adjustment}</p>
            </div>
        </div>
    );
};

export default DifficultyAdjustment;
