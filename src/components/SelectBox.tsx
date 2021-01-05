import React, { FC, useState } from 'react';
import { Select } from 'semantic-ui-react'

type selectOption = {
	key: string,
	value: string,
	text: string,
}
interface Props {
	value: string,
	options: Array<selectOption>,
	onChange: (e: any) => void
}

const SelectBox: FC<Props> = props => {
	const { value, options, onChange } = props;
	
	return (
		<>
		<select placeholder='5' value={value} onChange={onChange} >
		{ options.map((item, index) =>
			<option key={index}>
				{ item.value }
			</option>
			)
		}
		</select>
		</>
	)
}


export default SelectBox;
