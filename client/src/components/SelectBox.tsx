import React, { FC } from 'react';
import './SelectBox.css';

type selectOption = {
	key: number,
	value: number,
	text: string,
};
type Props = {
	value: number,
	options: Array<selectOption>,
	onChange: (e: React.ChangeEvent<HTMLSelectElement>) => void
};

const SelectBox: FC<Props> = props => {
	const { value, options, onChange } = props;
	
	return (
		<>
		<select className="select-box" value={value} onChange={onChange} >
		{ options.map((item, index) =>
			<option key={index}>
				{ item.text }
			</option>
			)
		}
		</select>
		</>
	);
};


export default SelectBox;
