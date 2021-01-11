import { FC } from 'react';
import { Card, Image } from 'semantic-ui-react';
import { resultType } from '../common/searchAPI';

type Props = {
	results: resultType[],
};

const SearchResult: FC<Props> = props => {
	const { results } = props;
	if (results.length > 0) {
		return (
			<Card.Group>
				{ results.map((item: any, index: number) =>
					<Card key={index} style={{ width: '328px' }} >
						<Card.Content>
							<Card.Header href={item.userLink} target="_blank" rel="noopener noreferrer" >
								<Image src={ item.profileImageUrl } floated='left' size='mini' />
								{ item.userName }@{ item.screenName }
							</Card.Header>
							<Card.Meta>
								<span className='date'>{ item.tweetTime }</span>
							</Card.Meta>
							<Card.Description href={ item.tweetLink } target="_blank" rel="noopener noreferrer" >
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
