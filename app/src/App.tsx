import React from 'react';
import { Routes, Route, Link } from 'react-router-dom';
import { Container, Navbar, Nav } from 'react-bootstrap';
import BlockHeightComponent from './BlockHeightComponent';
import BlockDetail from './BlockDetail';
import OffchainComponent from './OffchainComponent';
import RealTimeChart from './RealTimeChart';
import 'bootstrap/dist/css/bootstrap.min.css';

const App: React.FC = () => {
    return (
        <Container fluid>
            <Navbar bg="dark" expand="lg">
                <Navbar.Brand href="/">Blockchain Explorer</Navbar.Brand>
                <Navbar.Toggle aria-controls="basic-navbar-nav" />
                <Navbar.Collapse id="basic-navbar-nav">
                    <Nav className="ms-auto">
                        <Nav.Link as={Link} to="/">On-Chain Data</Nav.Link>
                        <Nav.Link as={Link} to="/offchain">Off-Chain Data</Nav.Link>
                        <Nav.Link as={Link} to="/realtime">Real-Time Chart</Nav.Link>
                    </Nav>
                </Navbar.Collapse>
            </Navbar>
            <Routes>
                <Route path="/" element={<BlockHeightComponent />} />
                <Route path="/block/:height" element={<BlockDetail />} />
                <Route path="/offchain" element={<OffchainComponent />} />
                <Route path="/realtime" element={<RealTimeChart />} />
            </Routes>
        </Container>
    );
};

export default App;
