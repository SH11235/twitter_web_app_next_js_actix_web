import React, { FC } from 'react';
import './SelectBox.css';

type selectOption = {
	key: string,
	value: string,
	text: string,
};
interface Props {
	value: string,
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
