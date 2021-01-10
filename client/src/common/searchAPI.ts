import { twitterBaseURL, searchAPIBaseURL } from './setting';

type conditions = {
	word: string,
	type: string,
}

export type resultsType = Required<{
	text: string,
	tweetLink: string,
	userLink: string,
	tweetTime: string,
	userName: string,
	screenName: string,
	profileImageUrl: string,
}>;

export const searchAPI = async (
	cond: conditions,
	setTotalPagesState: React.Dispatch<React.SetStateAction<number>>,
	setResultState: React.Dispatch<React.SetStateAction<{ results: resultsType[];}>>,
) => {
	try {
		const params: string[] = [`q=${cond.word}`, `type=${cond.type}`];
		const res = await fetch(`${searchAPIBaseURL}?${params.join('&')}`, {
			mode: 'cors'
		});
		const json = await res.json();
		setTotalPagesState(() => {
			// TODO 表示件数に合わせて変える
			return json.statuses.length;
		});
		const results: resultsType[] = json.statuses.map((item: any) => {
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
		setResultState(() => {
			return {
				results: results,
			};
		});
	} catch (error) {
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
				results: results,
			};
		});
	};
};

