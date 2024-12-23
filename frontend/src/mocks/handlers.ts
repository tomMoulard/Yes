import { rest } from 'msw';

export const handlers = [
  rest.post('/login', (req, res, ctx) => {
    const { email, password } = req.body;

    if (email === 'test@example.com' && password === 'password') {
      return res(
        ctx.status(200),
        ctx.json({ token: 'fake-jwt-token' })
      );
    }

    return res(
      ctx.status(401),
      ctx.json({ message: 'Unauthorized' })
    );
  }),

  rest.post('/register', (req, res, ctx) => {
    const { email, password, username } = req.body;

    if (email && password && username) {
      return res(
        ctx.status(200),
        ctx.json({ token: 'fake-jwt-token' })
      );
    }

    return res(
      ctx.status(400),
      ctx.json({ message: 'Bad Request' })
    );
  }),

  rest.post('/purchase', (req, res, ctx) => {
    const { points } = req.body;

    if (points > 0) {
      return res(
        ctx.status(200),
        ctx.json({ newBalance: points + 100 })
      );
    }

    return res(
      ctx.status(400),
      ctx.json({ message: 'Invalid points' })
    );
  })
];
