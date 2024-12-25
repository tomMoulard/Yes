import { server } from './mocks/server';
import '@testing-library/jest-dom';
import { beforeAll, afterAll, afterEach } from 'jest';

beforeAll(() => server.listen());
afterEach(() => server.resetHandlers());
afterAll(() => server.close());
