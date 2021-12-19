import React, { FC } from 'react';
import { Form, Radio } from 'semantic-ui-react'
import styles from '../styles/RadioButton.module.css';

type selectOption = {
	key: string,
	value: string,
	text: string,
};

type Props = {
	value: string,
	options: Array<selectOption>,
	onChange: (e: React.FormEvent<HTMLInputElement>, value: string) => void
};

const RadioButton: FC<Props> = props => {
	const { value, options, onChange } = props;
	
	return (
		<Form className={styles.radioForm}>
		{ options.map((item) =>
		 	<Form.Field className={styles.radioFormField}>
				 <Radio key={ item.key } label={item.text} value={ item.value } onChange={ (e) => onChange(e, item.value) } checked={ value === item.value } />
			 </Form.Field>
			)
		}
		</Form>
	);
};

export default RadioButton;
