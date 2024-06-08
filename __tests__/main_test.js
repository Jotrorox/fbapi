const request = require('supertest');
const express = require('express');
const app = require('../src/main');

describe('FizzBuzz API', () => {
    test('GET /fizzbuzz/:number with Fizz', async () => {
        const res = await request(app)
            .get('/fizzbuzz/3')
            .expect('Content-Type', /json/)
            .expect(200);

        expect(res.body).toEqual({ result: 'Fizz' });
    });

    test('GET /fizzbuzz/:number with Buzz', async () => {
        const res = await request(app)
            .get('/fizzbuzz/5')
            .expect('Content-Type', /json/)
            .expect(200);

        expect(res.body).toEqual({ result: 'Buzz' });
    });

    test('GET /fizzbuzz/:number', async () => {
        const res = await request(app)
            .get('/fizzbuzz/15')
            .expect('Content-Type', /json/)
            .expect(200);

        expect(res.body).toEqual({ result: 'FizzBuzz' });
    });

    test('GET /fizzbuzz/:number with non-multiple', async () => {
        const res = await request(app)
            .get('/fizzbuzz/17')
            .expect('Content-Type', /json/)
            .expect(200);

        expect(res.body).toEqual({ result: '17' });
    });

    test('GET /fizzbuzz/:number with non-number', async () => {
        const res = await request(app)
            .get('/fizzbuzz/abc')
            .expect('Content-Type', /json/)
            .expect(400);

        expect(res.body).toEqual({ error: 'Invalid number' });
    });

    test('GET /fizzbuzz/:number with negative number', async () => {
        const res = await request(app)
            .get('/fizzbuzz/-3')
            .expect('Content-Type', /json/)
            .expect(200);

        expect(res.body).toEqual({ result: '-3' });
    });
});