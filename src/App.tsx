import React, { FC, useState } from 'react';
import { Container, Divider, Header, Segment } from 'semantic-ui-react';
import SelectBox from './components/SelectBox';
import KeyWordBox from './components/KeyWordBox';
import SearchButton from './components/SearchButton';
import { Card, Image } from 'semantic-ui-react'


const valueOptions = [
	{ key: '5', value: '5', text: '5' },
	{ key: '10', value: '10', text: '10' },
	{ key: '15', value: '15', text: '15' },
	{ key: '30', value: '30', text: '30' },
];

// TODO 型付け
const r: any[] = [];

const App: FC = () => {
	const [ state, setState ] = useState({
		view: '5',
		word: 'テスト',
		results: r,
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
	const searchAPI = async () => {
		try {
			const res = await fetch('http://localhost:8000/twitter_search?q=' + state.word + '&count=' + state.view, {
				mode: 'cors'
			});
			const json = await res.json();
			const results = json.statuses.map((item: any) => {
				const twitterDomain = "https://twitter.com";
				const userLink = `${twitterDomain}/${item.user.screen_name}`;
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
	// searchAPI();

	return (
	<Container text style={{ marginTop: '7rem' }}>
		<Header as="h2">Twitter Search</Header>
		<Divider />
		<Segment>
			<Header as="h3">Search Conditions</Header>
			<KeyWordBox word={state.word} onChange={handleKeyWordChange}/>
			<SelectBox value={state.view} options={valueOptions} onChange={handleOptionChange}/>
			<SearchButton color="twitter" onClick={searchAPI} />
		</Segment>
		<Divider />
		<Header as="h2">Result</Header>
		<Divider />
		<Segment>
			<Card.Group>
			{ state.results.map((item: any, index: number) =>
				<Card key={index} >
					<Card.Content>
						<Card.Header href={item.userLink} >
							<Image src={ item.profileImageUrl } floated='left' size='mini' />
							{ item.userName }@{ item.screenName }
						</Card.Header>
						<Card.Meta>
							<span className='date'>{ item.tweetTime }</span>
						</Card.Meta>
						<Card.Description href={ item.tweetLink }>
							{ item.text }
						</Card.Description>
					</Card.Content>
					<Card.Content extra>
						
					</Card.Content>
				</Card>
				)
			}
			</Card.Group>
		</Segment>
	</Container>
	);
};

export default App;
