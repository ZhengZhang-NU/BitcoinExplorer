import React from 'react';
import { Routes, Route } from 'react-router-dom';
import BlockHeightComponent from './BlockHeightComponent';
import BlockDetail from './BlockDetail';
import DifficultyAdjustment from './DifficultyAdjustment';

const App: React.FC = () => {
    return (
        <Routes>
            <Route path="/" element={<BlockHeightComponent />} />
            <Route path="/block/:height" element={<BlockDetail />} />
            <Route path="/difficulty-adjustment" element={<DifficultyAdjustment />} />
        </Routes>
    );
};

export default App;
