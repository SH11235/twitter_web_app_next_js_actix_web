import React, { FC, useState, useEffect, useCallback } from 'react';
import { Container, Divider, Header, PaginationProps, Segment } from 'semantic-ui-react';
import SelectBox from './components/SelectBox';
import KeyWordBox from './components/KeyWordBox';
import SearchButton from './components/SearchButton';
import SearchResult from './components/SearchResult';
import RadioButton from './components/RadioButton';
import Pager from './components/Pager';
import {resultType, searchAPI} from './common/searchAPI';


const valueOptions = [
	{ key: 10, value: '10', text: '10' },
	{ key: 20, value: '20', text: '20' },
	{ key: 30, value: '30', text: '30' },
	{ key: 50, value: '50', text: '50' },
	{ key: 100, value: '100', text: '100' },
];

const radioOptions = [
	{ key: 'mixed', value: 'mixed', text: "mixed"},
	{ key: 'recent', value: 'recent', text: "recent"},
	{ key: 'popular', value: 'popular', text: "popular"},
];

const r: resultType[] = [];

const App: FC = () => {
	const [ searchCondState, setSearchCondState ] = useState({
		type: 'mixed',
	});

	let urlParamsStr = decodeURI(window.location.search);
	type paramsType = {
		word?: string,
	}
	let params: paramsType = {};
	if (urlParamsStr) {
		//?を除去
		urlParamsStr = urlParamsStr.substring(1);
		urlParamsStr.split('&').forEach( param => {
			const paramKeyVal = param.split('=');
			params = {
				...params,
				[paramKeyVal[0]]: paramKeyVal[1],
			};
		});
	}
	const [ keyWordState, setKeyWordState ] = useState(params.word ? params.word : 'テスト');

	const [ totalPagesState, setTotalPagesState ] = useState(0);

	const [ pageState, setPageState ] = useState({
		page: 1,
	});

	const [resultState, setResultState] = useState({
		results: r,
	});

	// keyWordStateを依存配列に加えるとキーワードが変更される度にAPIを叩かれてしまうため、意図的に外している。
	// この経緯で検索ボタンクリック時の処理は別名の関数として宣言する。
	const hitSearchAPI = useCallback(() => {
		const searchCond = {
			word: keyWordState,
			type: searchCondState.type,
		};
		searchAPI(searchCond, setTotalPagesState, setResultState);
		// eslint-disable-next-line react-hooks/exhaustive-deps
		}, [searchCondState.type]);

	const searchButtonClick = () => {
		const searchCond = {
			word: keyWordState,
			type: searchCondState.type,
		};
		searchAPI(searchCond, setTotalPagesState, setResultState);
	};

	useEffect(() => {
		hitSearchAPI();
		return;
		}, [hitSearchAPI]);
	
	const handleKeyWordChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		e.persist();
		const value = e.target.value;
		setKeyWordState(() => {
			return value;
		});
	};
	
	const [ viewNumState, setViewNumState ] = useState("10");
	const handleOptionChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
		e.persist();
		const value = e.target.value;
		setViewNumState(() => {
			return value;
		});
	};

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
			<SearchButton color="twitter" onClick={searchButtonClick} />
			<RadioButton value={searchCondState.type} options={radioOptions} onChange={handleRadioChange} />
		</Segment>
		<Divider />
		<Segment>
			<Header as="h3">Result: <SelectBox value={ viewNumState } options={valueOptions} onChange={handleOptionChange} />件／Page</Header>
			<SearchResult results={ resultState.results } />
			<Pager totalPages={ totalPagesState } onClick={ handlePageChange } />
		</Segment>
	</Container>
	);
};

export default App;
