import './App.css';
import React, { Suspense } from 'react';
import { Layout } from 'antd';
import Sider from 'antd/es/layout/Sider';
import { Footer } from 'antd/es/layout/layout';
import MenuComponent from './components/menu';

const { Content } = Layout;

const siderStyle: React.CSSProperties = {
  overflow: 'auto',
  height: '100vh',
  position: 'fixed',
  insetInlineStart: 0,
  top: 0,
  bottom: 0,
  scrollbarWidth: 'thin',
  scrollbarGutter: 'stable',
};

export default function App() {
  return (
	<Layout style={{ minHeight: '100vh' }}>
		<Sider
    breakpoint="lg"
    style={siderStyle}
		>
			<MenuComponent />

		</Sider>

		<Layout style={{ marginInlineStart: 200 }}>
			<Content style={{ padding: '0 50px', marginTop: 64, margin: '24px 16px 0', overflow: 'initial' }}>
				<Suspense fallback={
					<div>
						Loading...
					</div>
					}
				>
					{/* this is where to add future components to have the app built in*/}
				</Suspense>
			</Content>

			<Footer style={{ textAlign: 'center' }}>
				{'Yes © '}

				{new Date().getFullYear()}

				{' Created by Tom Corp.'}
			</Footer>
		</Layout>
	</Layout>
  );
}
