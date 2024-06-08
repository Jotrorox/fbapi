const express = require('express');
const multer = require('multer');
const fs = require('fs');
const { promisify } = require('util');
const readline = require('readline');

const app = express();
const upload = multer({ dest: 'uploads/' });

const unlinkAsync = promisify(fs.unlink);

app.use(express.json());

const fizzbuzz = (num) => {
    if (num < 0) return num.toString();
    if (num % 15 === 0) return 'FizzBuzz';
    if (num % 3 === 0) return 'Fizz';
    if (num % 5 === 0) return 'Buzz';
    return num.toString();
};

app.get('/fizzbuzz/:number', (req, res) => {
    const number = parseInt(req.params.number, 10);
    if (isNaN(number)) {
        return res.status(400).json({ error: 'Invalid number' });
    }
    const result = fizzbuzz(number);
    res.json({ result });
});

app.post('/fizzbuzz/file', upload.single('file'), async (req, res) => {
    const filePath = req.file.path;
    const results = [];
    
    try {
        const readInterface = readline.createInterface({
            input: fs.createReadStream(filePath),
            output: process.stdout,
            console: false
        });

        for await (const line of readInterface) {
            const number = parseInt(line, 10);
            if (!isNaN(number)) {
                results.push({ number, result: fizzbuzz(number) });
            }
        }

        res.json({ results });
    } catch (error) {
        res.status(500).json({ error: 'Error processing file' });
    } finally {
        try {
            await unlinkAsync(filePath);
        } catch (err) {
            console.error(`Error deleting file: ${err.message}`);
        }
    }
});

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
    console.log(`Server is running on port ${PORT}`);
});

module.exports = app;