import React, { FC } from 'react';
import { Form, Radio } from 'semantic-ui-react'

type selectOption = {
	key: string,
	value: string,
	text: string,
};

interface Props {
	value: string,
	options: Array<selectOption>,
	onChange: (e: React.FormEvent<HTMLInputElement>, value: string) => void
};

const RadioButton: FC<Props> = props => {
	const { value, options, onChange } = props;
	
	return (
		<Form>
		{ options.map((item) =>
		 	<Form.Field>
				 <Radio key={ item.key } label={item.text} value={ item.value } onChange={ (e) => onChange(e, item.value) } checked={ value === item.value } />
			 </Form.Field>
			)
		}
		</Form>
	);
};


export default RadioButton;
