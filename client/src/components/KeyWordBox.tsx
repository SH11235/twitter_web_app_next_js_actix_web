import React, { FC } from 'react';
import { Input } from 'semantic-ui-react';

interface Props {
	word: string,
	onChange: (e: React.ChangeEvent<HTMLInputElement>) => void
};

const KeyWordBox: FC<Props> = props => {
	const { word, onChange } = props;
	return (
		<>
		<Input placeholder="Search..." type="text" value={word} onChange={onChange} />
		</>
	);
};


export default KeyWordBox;
