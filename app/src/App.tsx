// App.tsx
import React from 'react';
import './App.css';
import BlockHeightComponent from './BlockHeightComponent';

const App: React.FC = () => {
    return (
        <div className="App">
            <nav className="navbar navbar-expand-lg navbar-dark bg-dark">
                <a className="navbar-brand" href="#">
                    My Block Explorer
                </a>
                <div className="collapse navbar-collapse" id="navbarNav">
                    <ul className="navbar-nav">
                        <li className="nav-item active">
                            <a className="nav-link" href="#">Home <span className="sr-only">(current)</span></a>
                        </li>
                    </ul>
                </div>
            </nav>
            <div className="container mt-5">
                <BlockHeightComponent />
            </div>
        </div>
    );
}

export default App;
