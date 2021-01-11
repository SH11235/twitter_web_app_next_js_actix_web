import { FC } from 'react';
import { Pagination, PaginationProps } from 'semantic-ui-react';


type Props = {
	pageState: {
		page: number,
		totalPages: number,
	},
	onClick: (e: PaginationProps) => void
};

const Pager: FC<Props> = props => {
	const { pageState, onClick } = props;
	return (
		<Pagination activePage={ pageState.page } totalPages={ pageState.totalPages } onClick={ onClick } />
	);
};

export default Pager;
