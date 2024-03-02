
CREATE or REPLACE FUNCTION mini1.select_v3(IN colname text, IN regexp text)
returns TABLE(id integer, name text, author text, publisher text, synopsis text, publish_date text)
LANGUAGE plpgsql as $$
BEGIN
 RETURN QUERY EXECUTE format('SELECT * FROM mini1."Books" WHERE %I ~ %L', colname, regexp);
END;
$$;

SELECT mini1.select_v3('author'::text,'\D+'::text);

SELECT * FROM mini1."Books" WHERE to_tsvector(synopsis) @@ to_tsquery('friend');
SELECT * FROM mini1."Books" WHERE 'author' ~ '\D+';
