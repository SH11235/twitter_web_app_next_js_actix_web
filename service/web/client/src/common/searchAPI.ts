import { searchAPIBaseURL } from './setting';

type conditions = {
	word: string,
	type: string,
	lang: string,
	count: string,
}

export type resultType = Required<{
	text: string,
	tweet_link: string,
	user_link: string,
	tweet_time: string,
	user_name: string,
	screen_name: string,
	profile_image_url: string,
}>;

type pageState = {
	page: number,
	totalPages: number,
	view: number,
}

export const searchAPI = async (
	cond: conditions,
	pageState: pageState,
	setPageState: React.Dispatch<React.SetStateAction<{
		page: number;
		totalPages: number;
		view: number;
	}>>,
	resultState: {
		results: resultType[];
		count: number;
	},
	setResultState: React.Dispatch<React.SetStateAction<{
		results: resultType[];
		count: number;
	}>>,
) => {
	try {
		const params: string[] = [`q=${cond.word}`, `type=${cond.type}`, `lang=${cond.lang}`, `count=${cond.count}`];
		const res = await fetch(`${searchAPIBaseURL}?${params.join('&')}`, {
			mode: 'cors'
		});
		let results: resultType[] = await res.json();
		const count = results.length;
		if (results.length > 0) {
			setPageState(() => {
				const totalPages = Math.ceil((count - 1) / pageState.view);
				return {
					...pageState,
					page: 1,
					totalPages: totalPages,
				};
			});
		} else {
			results = [{
				text: "No Results. Change the search conditions.",
				tweet_link: "",
				user_link: "",
				tweet_time: "",
				user_name: "anyone",
				screen_name: "id",
				profile_image_url: "",
			}];
		}

		setResultState(() => {
			return {
				...resultState,
				results: results,
				count: count,
			};
		});
	} catch (error) {
		console.error(error);
		const results = [{
			text: "アクセス制限中",
			tweet_link: "",
			user_link: "",
			tweet_time: "",
			user_name: "",
			screen_name: "",
			profile_image_url: "",
		}];
		setResultState(() => {
			return {
				...resultState,
				results: results,
				count: 0,
			};
		});
	};
};

