import { FC } from 'react';
import { Card, Image } from 'semantic-ui-react';

type Props = {
	results: any[],
};

const SearchResult: FC<Props> = props => {
	const { results } = props;
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
};

export default SearchResult;
