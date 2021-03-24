import { twitterBaseURL, searchAPIBaseURL } from './setting';

type conditions = {
	word: string,
	type: string,
}

export type resultType = Required<{
	text: string,
	tweetLink: string,
	userLink: string,
	tweetTime: string,
	userName: string,
	screenName: string,
	profileImageUrl: string,
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
		const params: string[] = [`q=${cond.word}`, `type=${cond.type}`];
		const res = await fetch(`${searchAPIBaseURL}?${params.join('&')}`, {
			mode: 'cors'
		});
		const json = await res.json();
		const count = json.length;
		let results: resultType[];
		if (json.length > 0) {
			setPageState(() => {
				const totalPages = Math.ceil((count - 1) / pageState.view);
				return {
					...pageState,
					page: 1,
					totalPages: totalPages,
				};
			});
			results = json.map((item: any) => {
				const userLink = `${twitterBaseURL}/${item.user.screen_name}`;
				const tweetLink = `${userLink}/status/${item.id_str}`;
				return {
					text: item.text,
					tweetLink: tweetLink,
					userLink: userLink,
					tweetTime: item.created_at,
					userName: item.user.name,
					screenName: item.user.screen_name,
					profileImageUrl: item.user.profile_image_url_https,
				};
			});
		} else {
			results = [{
				text: "No Results. Change the search conditions.",
				tweetLink: "",
				userLink: "",
				tweetTime: "",
				userName: "anyone",
				screenName: "id",
				profileImageUrl: "",
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
			tweetLink: "",
			userLink: "",
			tweetTime: "",
			userName: "",
			screenName: "",
			profileImageUrl: "",
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

