import React, { FC, useState } from 'react';
import { Container, Divider, Header, PaginationProps, Segment } from 'semantic-ui-react';
import SelectBox from './components/SelectBox';
import KeyWordBox from './components/KeyWordBox';
import SearchButton from './components/SearchButton';
import SearchResult from './components/SearchResult';
import RadioButton from './components/RadioButton';
import Pager from './components/Pager';
import { twitterBaseURL, searchAPIBaseURL } from './common/setting';


const valueOptions = [
	{ key: '5', value: '5', text: '5' },
	{ key: '10', value: '10', text: '10' },
	{ key: '15', value: '15', text: '15' },
	{ key: '30', value: '30', text: '30' },
];

const radioOptions = [
	{  key: 'mixed', value: 'mixed', text: "mixed"},
	{  key: 'recent', value: 'recent', text: "recent"},
	{  key: 'popular', value: 'popular', text: "popular"},
];

// TODO 型付け
const r: any[] = [];

const App: FC = () => {
	const [ state, setState ] = useState({
		view: '5',
		word: 'テスト',
		type: 'mixed',
		results: r,
	});

	const [ pageState, setPageState ] = useState({
		page: 1,
		totalPages: 0,
	});

	const handleKeyWordChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		e.persist();
		const value = e.target.value;
		setState(() => {
			return {...state, word: value };
		});
	};
	
	const handleOptionChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
		e.persist();
		const value = e.target.value;
		setState(() => {
			return {...state, view: value };
		});
	};

	const handleRadioChange = (e: React.FormEvent<HTMLInputElement>, value: string) => {
		e.persist();
		setState(() => {
			return {...state, type: value };
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
			page = pageState.page === pageState.totalPages ? pageState.totalPages : pageState.page + 1;
		} else if (pager === "»") {
			page = pageState.totalPages;
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

	const searchAPI = async () => {
		try {
			const params: string[] = [`q=${state.word}`, `count=${state.view}`, `type=${state.type}`];
			const res = await fetch(`${searchAPIBaseURL}?${params.join('&')}`, {
				mode: 'cors'
			});
			const json = await res.json();
			setPageState(() => {
				console.log("json");
				console.log(json.statuses.length);
				return {
					...pageState,
					totalPages: json.statuses.length,
				};
			});
			const results = json.statuses.map((item: any) => {
				const userLink = `${twitterBaseURL}/${item.user.screen_name}`;
				const tweetLink = `${userLink}/status/${item.id_str}`;
				return {
					text: item.text,
					tweetLink: tweetLink,
					userLink: userLink,
					tweetTime: item.created_at,
					userName: item.user.name,
					screenName: item.user.screen_name,
					profileImageUrl: item.user.profile_image_url_https,
				};
			});
			setState(() => {
				return {
					...state,
					results: results,
				};
			});
		} catch (error) {
			const results = [{
				text: "アクセス制限中",
				tweetLink: "",
				userLink: "",
				tweetTime: "",
				userName: "",
				screenName: "",
				profileImageUrl: "",
			}];
			setState(() => {
				return {
					...state,
					results: results,
				};
			});
		};
	};

	return (
	<Container text style={{ marginTop: '7rem' }}>
		<Header as="h2">Twitter Search</Header>
		<Divider />
		<Segment>
			<Header as="h3">Search Conditions</Header>
			<KeyWordBox word={state.word} onChange={handleKeyWordChange} />
			<SelectBox value={state.view} options={valueOptions} onChange={handleOptionChange} />
			<SearchButton color="twitter" onClick={searchAPI} />
			<RadioButton value={state.type} options={radioOptions} onChange={handleRadioChange} />
		</Segment>
		<Divider />
		<Header as="h2">Result</Header>
		<Divider />
		<Segment>
			<SearchResult results={state.results} />
			<Pager totalPages={ pageState.totalPages } onClick={ handlePageChange } />
		</Segment>
	</Container>
	);
};

export default App;
