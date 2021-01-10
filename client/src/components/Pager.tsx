import { FC } from 'react';
import { Pagination, PaginationProps } from 'semantic-ui-react';

type Props = {
	totalPages: number,
	onClick: (e: PaginationProps) => void
};

const Pager: FC<Props> = props => {
	const { totalPages, onClick } = props;
	return (
		<Pagination totalPages={ totalPages } onClick={ onClick } />
	);
};

export default Pager;
