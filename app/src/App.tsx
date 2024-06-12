import React from "react";
import { Routes, Route } from "react-router-dom";
import BlockHeightComponent from "./BlockHeightComponent";
import BlockDetail from "./BlockDetail";
import TransactionFees from "./TransactionFees";
import DifficultyAdjustment from "./DifficultyAdjustment";
import MempoolGoggles from "./MempoolGoggles";

const App: React.FC = () => {
    return (
        <Routes>
            <Route path="/" element={<BlockHeightComponent />} />
            <Route path="/block/:height" element={<BlockDetail />} />
            <Route path="/transaction-fees" element={<TransactionFees />} />
            <Route path="/difficulty-adjustment" element={<DifficultyAdjustment />} />
            <Route path="/mempool-goggles" element={<MempoolGoggles />} />
        </Routes>
    );
};

export default App;
