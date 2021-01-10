import React, { FC, useState, useEffect } from 'react';
import { Container, Divider, Header, PaginationProps, Segment } from 'semantic-ui-react';
import SelectBox from './components/SelectBox';
import KeyWordBox from './components/KeyWordBox';
import SearchButton from './components/SearchButton';
import SearchResult from './components/SearchResult';
import RadioButton from './components/RadioButton';
import Pager from './components/Pager';
import {resultsType, searchAPI} from './common/searchAPI';


// const valueOptions = [
// 	{ key: '5', value: '5', text: '5' },
// 	{ key: '10', value: '10', text: '10' },
// 	{ key: '15', value: '15', text: '15' },
// 	{ key: '30', value: '30', text: '30' },
// ];

const radioOptions = [
	{  key: 'mixed', value: 'mixed', text: "mixed"},
	{  key: 'recent', value: 'recent', text: "recent"},
	{  key: 'popular', value: 'popular', text: "popular"},
];

const r: resultsType[] | [] = [];

const App: FC = () => {
	const [ searchCondState, setSearchCondState ] = useState({
		type: 'mixed',
	});

	const [ keyWordState, setKeyWordState ] = useState('テスト');

	const [ totalPagesState, setTotalPagesState ] = useState(0);

	const [ pageState, setPageState ] = useState({
		page: 1,
	});

	const [resultState, setResultState] = useState({
		results: r,
	});

	useEffect(() => {
		const searchCond = {
			word: keyWordState,
			type: searchCondState.type,
		}
		searchAPI(searchCond, setTotalPagesState, setResultState);
		return;
		// eslint-disable-next-line react-hooks/exhaustive-deps
		}, [searchCondState.type]);

	const handleKeyWordChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		e.persist();
		const value = e.target.value;
		setKeyWordState(() => {
			return value;
		});
	};
	
	// TODO 表示件数指定のセレクトボックスを作る際にこれをもとにする
	// const handleOptionChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
	// 	e.persist();
	// 	const value = e.target.value;
	// 	setState(() => {
	// 		return {...state, view: value };
	// 	});
	// };

	const handleRadioChange = (e: React.FormEvent<HTMLInputElement>, value: string) => {
		e.persist();
		setSearchCondState(() => {
			return {...searchCondState, type: value };
		});
	};

	const handlePageChange = (e: PaginationProps) => {
		e.persist();
		const pager = e.target.text;
		const pagerToInt = parseInt(pager);
		let page = pageState.page;
		if (pager === "⟨") {
			page = pageState.page === 1 ? 1 : pageState.page - 1;
		} else if (pager === "«") {
			page = 1;
		} else if (pager === "⟩") {
			page = pageState.page === totalPagesState ? totalPagesState : pageState.page + 1;
		} else if (pager === "»") {
			page = totalPagesState;
		} else if (pager === "...") {
			page = pageState.page;
		} else if (Number.isInteger(pagerToInt)) {
			page = pagerToInt;
		}
		setPageState(() => {
			return {
				...pageState,
				page: page,
			};
		});
	};

	return (
	<Container text style={{ marginTop: '7rem' }}>
		<Header as="h2">Twitter Search</Header>
		<Divider />
		<Segment>
			<Header as="h3">Search Conditions</Header>
			<KeyWordBox word={keyWordState} onChange={handleKeyWordChange} />
			{/* // TODO 表示件数指定のセレクトボックスを作る際にこれをもとにする
				<SelectBox value={state.view} options={valueOptions} onChange={handleOptionChange} /> 
			*/}
			{/* <SearchButton color="twitter" onClick={searchAPI} /> */}
			<RadioButton value={searchCondState.type} options={radioOptions} onChange={handleRadioChange} />
		</Segment>
		<Divider />
		<Header as="h2">Result</Header>
		<Divider />
		<Segment>
			<SearchResult results={resultState.results} />
			<Pager totalPages={ totalPagesState } onClick={ handlePageChange } />
		</Segment>
	</Container>
	);
};

export default App;
