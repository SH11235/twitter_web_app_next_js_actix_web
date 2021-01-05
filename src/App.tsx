import React, { FC, useState } from 'react';
import { Container, Divider, Header, Segment, Input } from 'semantic-ui-react'
import './App.css';
import SelectBox from './components/SelectBox';

const valueOptions = [
	{ key: '5', value: '5', text: '5' },
	{ key: '10', value: '10', text: '10' },
	{ key: '15', value: '15', text: '15' },
	{ key: '30', value: '30', text: '30' },
];

const App: FC = () => {
	const [ state, setState ] = useState({
		view: '5',
	});
	
	const handleOptionChange = (e: any) => {
		e.persist();
		const value = e.target.value;
		console.log(value);
		setState(() => {
			return {...state, view: value };
		});
	};

	return (
	<Container text style={{ marginTop: '7rem' }}>
		<Header as="h2">Twitter Search</Header>
		<Divider />
		<Segment>
		<Header as="h3">キーワードボックス, 表示件数</Header>
		<Input placeholder='Search...' />
		<SelectBox value={state.view} options={valueOptions} onChange={handleOptionChange}/>
		</Segment>
	</Container>
	)
};

export default App;
