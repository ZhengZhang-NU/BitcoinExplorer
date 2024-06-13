import React from 'react';
import { Routes, Route } from 'react-router-dom';
import BlockHeightComponent from './BlockHeightComponent';
import BlockDetail from './BlockDetail';


const App: React.FC = () => {
    return (
        <Routes>
            <Route path="/" element={<BlockHeightComponent />} />
            <Route path="/block/:height" element={<BlockDetail />} />
        </Routes>
    );
};

export default App;
