import './index.css';
import App from './App';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { ConfigProvider } from 'antd';

const root = ReactDOM.createRoot(document.getElementById('root')!);

root.render(
	<React.StrictMode>
		<ConfigProvider>
			<App />
		</ConfigProvider>
	</React.StrictMode>
);
