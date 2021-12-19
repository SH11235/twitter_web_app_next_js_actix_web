import { FC } from 'react';
import { Card, Image, Icon } from 'semantic-ui-react';
import { resultType } from '../common/searchAPI';
import { tweetDBRequestBaseURL } from '../common/setting';
import styles from '../styles/SearchResult.module.css';

type Props = {
	results: resultType[],
	page: number,
	view: number,
};

const SearchResult: FC<Props> = props => {
	const { results, page, view } = props;
	const startIndex = (page - 1) * view;
	const endIndex = page * view;
	const filterResults = results.filter((result, index) => {
		return (index >= startIndex) && (index < endIndex);
	});
	const postTweet = (e: React.MouseEvent<HTMLElement>) => {
		if (e.currentTarget.dataset.item) {
			const item: resultType = JSON.parse(e.currentTarget.dataset.item);
			console.log(item);
			const requestOptions ={
				method: 'POST',
				headers:{'Content-Type': 'application/json'},
				body: JSON.stringify(item)
			};
			fetch(tweetDBRequestBaseURL, requestOptions)
				.then(() => {
					alert("Success for registering a record.")
				});
		}
	};
	
	if (filterResults.length > 0) {
		return (
			<Card.Group>
				{ filterResults.map((item: resultType, index: number) =>
					<Card key={index} style={{ width: '328px' }} >
						<Card.Content>
							<Card.Header>
								<a href={item.user_link} target="_blank" rel="noopener noreferrer">
									<Image src={ item.profile_image_url } floated='left' size='mini' />
									{ item.user_name }@{ item.screen_name }
								</a>
								<button className={ styles.favIcon } onClick={ postTweet } data-item={JSON.stringify(item)} >
									<Icon name="favorite" color='yellow' />
								</button>
							</Card.Header>
							<Card.Meta>
								<span className='date'>{ item.tweet_time }</span>
							</Card.Meta>
							<Card.Description href={ item.tweet_link } target="_blank" rel="noopener noreferrer" >
								{ item.text }
							</Card.Description>
						</Card.Content>
						<Card.Content extra>
							
						</Card.Content>
					</Card>
					)
				}
			</Card.Group>
		);
	} else {
		return (
			<Card.Group>
				<Card style={{ width: '328px' }} >
					<Card.Content>
						<Card.Description>
							No Results. Set chenge search conditions.
						</Card.Description>
					</Card.Content>
				</Card>
			</Card.Group>
		);
	}
};

export default SearchResult;
