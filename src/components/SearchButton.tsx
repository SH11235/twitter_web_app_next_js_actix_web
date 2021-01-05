import React, { FC } from 'react';
import { Button } from 'semantic-ui-react'

type ButtonColor = "red" | "orange" | "yellow" | "olive" | "green" | "teal" | "blue" | "violet" | "purple" | "pink" | "brown" | "grey" | "black" | "facebook" | "google plus" | "vk" | "twitter" | "linkedin" | "instagram" | "youtube" | undefined;

interface Props {
	color: ButtonColor,
	onClick: () => void
};

const SearchButton: FC<Props> = props => {
	const { color, onClick } = props;
	return (
		<>
		<Button color={color} onClick={onClick}>Search</Button>
		</>
	);
};

export default SearchButton;
