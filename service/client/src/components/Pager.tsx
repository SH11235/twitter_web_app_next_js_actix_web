import { FC } from 'react';
import { Pagination, PaginationProps } from 'semantic-ui-react';
import styles from '../styles/Pager.module.css'

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
		<Pagination className={ styles.pagination } activePage={ pageState.page } totalPages={ pageState.totalPages } onClick={ onClick } />
	);
};

export default Pager;
