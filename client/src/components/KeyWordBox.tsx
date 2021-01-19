import React, { FC } from 'react';
import { Input } from 'semantic-ui-react';

type Props = {
	word: string,
	onChange: (e: React.ChangeEvent<HTMLInputElement>) => void
	onKeyPress: (e: React.KeyboardEvent<HTMLInputElement>) => void,
};

const KeyWordBox: FC<Props> = props => {
	const { word, onChange, onKeyPress } = props;
	return (
		<>
		<Input placeholder="Search..." type="text" value={word} onChange={onChange} onKeyPress={onKeyPress}/>
		</>
	);
};


export default KeyWordBox;
