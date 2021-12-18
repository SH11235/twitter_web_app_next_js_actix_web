import { FC } from 'react';
import { Card, Image } from 'semantic-ui-react';
import { resultType } from '../common/searchAPI';

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
	if (filterResults.length > 0) {
		return (
			<Card.Group>
				{ filterResults.map((item: resultType, index: number) =>
					<Card key={index} style={{ width: '328px' }} >
						<Card.Content>
							<Card.Header href={item.user_link} target="_blank" rel="noopener noreferrer" >
								<Image src={ item.profile_image_url } floated='left' size='mini' />
								{ item.user_name }@{ item.screen_name }
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
