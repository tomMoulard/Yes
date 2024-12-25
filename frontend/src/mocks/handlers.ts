import { http, HttpResponse } from 'msw';

export const handlers = [
	http.post('/login', ({ request }) => {
		console.log("mock /login", {request});

		// if (email === 'test@example.com' && password === 'password') {
			// return HttpResponse.json({ token: 'fake-jwt-token' })
		// }

		return new HttpResponse(null, { status: 401 });
	}),

	http.post('/register', ({ request }) => {
		console.log("mock /register", {request});

		// if (email && password && username) {
			// return HttpResponse.json({ token: 'fake-jwt-token' })
		// }

		return new HttpResponse(null, { status: 401 });
	}),

	http.post('/purchase', ({ request }) => {
		console.log("mock /purchase", {request});
		// const { points } = req.body;

		// if (points > 0) {
				// ctx.json({ newBalance: points + 100 })
			// return HttpResponse.json({ token: 'fake-jwt-token' })
		// }

		return new HttpResponse(null, {
			status: 400,
			statusText: 'Invalid points',
		});
	})
];

